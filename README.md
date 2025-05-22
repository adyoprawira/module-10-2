# Module 10-2

#### Three clients and one server screenshot

![Three clients and one server screenshot](https://github.com/user-attachments/assets/07ad0161-ec95-4c83-ae8b-d88294b7571e)


#### How to run
1. Run one server terminal by using `cargo run --bin server`.
2. Run four client terminal by using `cargo run --bin client`.
3. Send a message across all clients.

#### Explanation
As can be seen from the screenshot above, when a client sends a message, all the clients will receive the message from the server, including the sender. The server acts as a middle service that receives and sends messages throughout the clients. It will remember each client that is connected to it and will wait for any messages.