----- 00 视频介绍 ----- ----- ----- ----- -----
很多教材或多或少都需要一些编程的功底，否则难以很好理解抽象的功能
在这个视频中，我会使用更加多的描述与动画来让学习变得更具体
让这个视频更加适合完全没有学习过编程的人

视频中也或多或少有没能涉及到的可能出错的地方，这个时候可以自行搜索或发在评论中

科学上网，chatgpt（可选）

----- 01 展示风格 ----- ----- ----- ----- -----
总章节---本视频章节



----- 02 准备工作 ----- ----- ----- ----- -----
官方网站
https://www.rust-lang.org/zh-CN/

安装 Rust
https://www.rust-lang.org/zh-CN/tools/install

创建项目
>>> cargo new project_name

IDE + 插件
推荐homebrew与scoop (可选)
vim, helix, vs code, rustrover, zed

zed
自动cargo fmt
lsp


没有网时本地二进制解决办法
chmod +x

github管理(可选)
注册自己的github
使用github desktop



----- 03 rust项目特性 ----- ----- ----- ----- -----
cargo
.toml .lock
cargo run | cargo run --release 优化

使用test
#[test]
cargo test | cargo test --release

文件夹管理
mod, create, super

target
target产物过大, 时间长，cargo check，，尽可能在同一个文件中写代码
debug模式下，一个surrealdb有2.2G
release模式下，一个surrealdb有0.9G

可执行文件
cargo build | cargo build --release

gitignore
