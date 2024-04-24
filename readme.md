# rust_telemetry_tutorial

## 链路追踪
![](./doc/trace.jpg)

## 示例数据流
![](./doc/tracing.jpg)
1. 使用[tracing](https://github.com/tokio-rs/tracing)规范化生成trace的span信息。 
2. 通过 subscriber 来处理和展示span信息。

## connecter
![](./doc/connecter.jpg)

## 运行
默认执行路径为当前项目下
1. 运行测试环境
```
docker-compose up -d
...
```
2. 执行测试程序
注：这里执行rust 程序需要切换到rust_telemetry_tutorial目录下
```
cd rust_telemetry_tutorial
cargo run run src/main.rs
```
3. 执行结果
![](./doc/result.png)

## 参考:  
[tracing](https://github.com/tokio-rs/tracing)  
[opentelemetry-collector](https://github.com/open-telemetry/opentelemetry-collector)  
[exampleconnector](https://github.com/gord02/exampleconnector)