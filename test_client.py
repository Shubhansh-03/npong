import asyncio
import websockets
import json

async def main():
    async with websockets.connect("ws://127.0.0.1:8080/ws") as websocket:
        player_id = await websocket.recv()
        print(f"Connected as Player {player_id}")
        
        # Send a dummy message
        await websocket.send(json.dumps({"paddle_x": 100.5}))
        
        for _ in range(5):
            msg = await websocket.recv()
            print(f"Received: {msg}")

asyncio.run(main())
