const socket = new WebSocket(`ws://${location.host}/ws`);
const form = document.getElementById("chat-form");
const messageBoard = document.getElementById("message-board");
const textInput = document.getElementById("chat-input");

const BOTTOM_EPS = 16;

function isAtBottom(el) {
    return el.scrollTop + el.clientHeight >= el.scrollHeight - BOTTOM_EPS;
}

function scrollToBottom(el) {
    el.scrollTop = el.scrollHeight;
}

form.addEventListener("submit", (e) => {
    e.preventDefault();
    if (socket.readyState !== WebSocket.OPEN) return;

    const msg = {
        type: "chat",
        text: textInput.value
    };

    socket.send(JSON.stringify(msg));
    textInput.value = "";
    textInput.focus();
});

socket.onmessage = (event) => {
    const msg = JSON.parse(event.data);

    const wasAtBottom = isAtBottom(messageBoard);

    if (msg.type === "chat") {
        const newMessage = document.createElement("tr");
        newMessage.innerHTML = `<b>${msg.username}:</b> ${msg.text}`;
        messageBoard.appendChild(newMessage);
    } else if (msg.type === "system") {
        const newMessage = document.createElement("tr");
        newMessage.innerHTML = `<b>System:</b> ${msg.message}`;
        messageBoard.appendChild(newMessage);
    }

    if (wasAtBottom) {
        scrollToBottom(messageBoard);
    }
};