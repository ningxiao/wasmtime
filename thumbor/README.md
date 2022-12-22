# 编译 thumbor 项目
反向代理拉去图片进行水印
cargo build --release
# 运行日志
RUST_LOG=info target/release/thumbor
# 测试链接
http://localhost:8080/image/CgoKCAj0AxCgBiADCgY6BAgUEBQKBDICCAM/https%3A%2F%2Fimages%2Epexels%2Ecom%2Fphotos%2F1562477%2Fpexels%2Dphoto%2D1562477%2Ejpeg%3Fauto%3Dcompress%26cs%3Dtinysrgb%26w%3D1260%26h%3D750%26dpr%3D2