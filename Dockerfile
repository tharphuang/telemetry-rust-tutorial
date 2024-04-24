FROM golang:1.21.0 AS builder
ENV CGO_ENABLED=0
ENV GOOS=linux
ENV GOARCH=amd64
ENV GOPROXY=https://goproxy.cn

WORKDIR /build
COPY ./otel_connector /build/
RUN go build -ldflags="-s -w" -o /build/otel /build/main.go

FROM alpine AS final
RUN apk update --no-cache && apk add --no-cache ca-certificates tzdata
ENV TZ Asia/Shanghai
WORKDIR /app
COPY --from=builder /build/otel /app/otel
CMD ["/app/otel","--config","/app/collet-config.yaml"]