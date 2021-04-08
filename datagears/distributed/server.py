import logging
import asyncio
import grpc

from datagears.commons.protos.datagears_pb2_grpc import DataGearsServicer, DataGearsStub, add_DataGearsServicer_to_server
from datagears.commons.protos.datagears_pb2 import Ping, Pong


class DGService(DataGearsServicer):
    """Datagears backend service."""

    async def SayHello(self, request: Ping, context: grpc.aio.ServicerContext) -> Pong:
        """Ping-Pong endpoint."""
        return Pong(message='Hello, %s!' % request.name)


async def serve() -> None:
    """Start server."""
    server = grpc.aio.server()
    add_DataGearsServicer_to_server(DGService(), server)

    listen_addr = '[::]:50051'
    server.add_insecure_port(listen_addr)

    logging.info("Starting server on %s", listen_addr)
    await server.start()
    
    try:
        await server.wait_for_termination()
    except KeyboardInterrupt:
        # Shuts down the server with 0 seconds of grace period. During the
        # grace period, the server won't accept new connections and allow
        # existing RPCs to continue within the grace period.
        await server.stop(0)


if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)
    asyncio.run(serve())
