const socket = new WebSocket("ws://localhost:3000/ws");
const form = document.getElementById("chat-form");
const messageBoard = document.getElementById("message-board");
const textInput = document.getElementById("chat-input");
const userInput = document.getElementById("name-input");

form.addEventListener("submit", (e) => {
    e.preventDefault();
    if (socket.readyState !== WebSocket.OPEN) return;

    const msg = {
        type: "chat",
        user: userInput.value || "Anonymous",
        text: textInput.value
    };

    socket.send(JSON.stringify(msg));
    textInput.value = "";
    textInput.focus();
});



socket.onmessage = (event) => {
    const msg = JSON.parse(event.data);
    if (msg.type === "chat") {
        const newMessage = document.createElement("tr");
        newMessage.textContent = msg.text;
        messageBoard.appendChild(newMessage);
    } else if (msg.type === "system") {
        const newMessage = document.createElement("tr");
        newMessage.textContent = msg.text;
        messageBoard.appendChild(newMessage);
    }
};