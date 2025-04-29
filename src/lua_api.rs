use anyhow::Result;
use mlua::{AnyUserData, Lua, Table, UserData, UserDataMethods};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::runtime::Runtime;

use crate::replace_strings::{Config, Replacement};

#[derive(Clone)]
struct LuaConfig {
    config: Arc<Config>,
}

impl UserData for LuaConfig {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_recursive", |_, this, val: bool| {
            Arc::make_mut(&mut this.config).recursive = val;
            Ok(())
        });
        methods.add_method_mut("set_invert", |_, this, val: bool| {
            Arc::make_mut(&mut this.config).invert = val;
            Ok(())
        });
        methods.add_method_mut("set_filter", |_, this, val: Option<String>| {
            Arc::make_mut(&mut this.config).filter = val;
            Ok(())
        });
    }
}

pub fn init(lua: &Lua) -> Result<()> {
    let module = lua.create_table()?;

    module.set(
        "init",
        lua.create_function(|lua, config: Table| {
            let mut changes = HashMap::new();

            for pair in config.pairs::<String, mlua::Value>() {
                let (key, val) = pair?;
                let replacement = match val {
                    mlua::Value::String(s) => Replacement::Exact(s.to_str()?.to_string()),
                    mlua::Value::Table(t) => Replacement::Regex {
                        pattern: t.get("pattern")?,
                        replacement: t.get("replacement")?,
                    },
                    _ => {
                        return Err(mlua::Error::RuntimeError(format!(
                            "Invalid replacement type for key: {}",
                            key
                        )));
                    }
                };
                changes.insert(key, replacement);
            }

            let lua_config = LuaConfig {
                config: Arc::new(Config {
                    changes,
                    recursive: false,
                    filter: None,
                    invert: false,
                }),
            };

            lua.globals().set("__lua_config", lua_config)?;
            Ok(())
        })?,
    )?;

    module.set(
        "run",
        lua.create_function(|lua_ctx, path: String| {
            let ud: AnyUserData = lua_ctx.globals().get("__lua_config")?;
            let lua_cfg_ref = ud.borrow::<LuaConfig>()?;
            let cfg = lua_cfg_ref.config.clone();
            let rt = Runtime::new()?;

            let results = rt.block_on(async {
                let mut processed = Vec::new();
                let args = crate::RunArgs {
                    input_path: PathBuf::from(&path),
                    output_file: None,
                    config_path: PathBuf::from(""),
                    verbose: false,
                    dry_run: false,
                    backup: false,
                    interactive: false,
                    recursive: cfg.recursive,
                    filter: cfg.filter.clone(),
                    invert: cfg.invert,
                };

                if let Err(e) = crate::process_recursive(&args, &cfg).await {
                    processed.push(format!("Error: {:?}", e));
                }

                processed
            });

            Ok(results)
        })?,
    )?;

    lua.globals().set("filesculptor", module)?;
    Ok(())
}
