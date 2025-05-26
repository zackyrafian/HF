"use client";
import { useEffect, useState } from "react";

export default function Chat() {
  const [user, setUser] = useState("");
  const [message, setMessage] = useState("");
  const [inputMessage, setInputMessage] = useState("");
  const [receiverId, setReceiverId] = useState("");
  const ws = new WebSocket("ws://localhost:4000/ws");

  useEffect(() => {
    ws.onmessage = (e) => {
      setMessage(e.data);
    };

    ws.onopen = () => {
      setUser(Math.random().toString(36).substr(2, 9));
    };

    return () => {
      ws.close();
    };
  }, []);

  const sendMessage = () => {
    if (inputMessage.trim() && receiverId.trim()) {
      ws.send(
        JSON.stringify({
          sender: user,
          receiver: receiverId,
          message: inputMessage,
        })
      );
      setInputMessage("");
    }
  };

  return (
    <div>
      <h1>WebSocket Message: {message}</h1>
      <div>
        <input
          type="text"
          value={inputMessage}
          onChange={(e) => setInputMessage(e.target.value)}
          placeholder="Type a message..."
        />
        <input
          type="text"
          value={receiverId}
          onChange={(e) => setReceiverId(e.target.value)}
          placeholder="Receiver's ID"
        />
        <button onClick={sendMessage}>Send</button>
      </div>
    </div>
  );
}
