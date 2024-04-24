# rust_telemetry_tutorial

## 链路追踪
![](./doc/trace.jpg)

## 示例数据流
![](./doc/tracing.jpg)
1. 使用[tracing](https://github.com/tokio-rs/tracing)规范化生成trace的span信息。 
2. 通过 subscriber 来处理和展示span信息。

## connecter
![](./doc/connecter.jpg)


参考:
[tracing](https://github.com/tokio-rs/tracing)  
[opentelemetry-collector](https://github.com/open-telemetry/opentelemetry-collector)  
[exampleconnector](https://github.com/gord02/exampleconnector)