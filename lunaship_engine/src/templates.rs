use askama::Template;

#[derive(Template)]
#[template(path = "common/gitignore")]
pub struct GitIgnore;

#[derive(Template)]
#[template(path = "common/lunaship_toml")]
pub struct LunashipProject {
    pub project_name: String,
}

#[derive(Template)]
#[template(path = "love/lua_rc")]
pub struct LoveLuaRc;

#[derive(Template)]
#[template(path = "love/main")]
pub struct LoveMain {
    pub project_name: String,
}
