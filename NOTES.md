

Generate GRPC files:
```
python -m grpc_tools.protoc -I./datagears/commons/protos --python_out=./datagears/commons/protos --grpc_python_out=./datagears/commons/protos ./datagears/commons/protos/datagears.proto
```