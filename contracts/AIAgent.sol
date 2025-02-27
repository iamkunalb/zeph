// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract AIAgent {
    string public message = "Hi bitch";
    string public response;


    // Event to emit when a new message is sent
    event MessageSent(string newMessage);
    event ResponseUpdated(string newResponse);

    // Function to send a message
    function sendMessage(string calldata _message) external {
        message = _message; // Store the message
        emit MessageSent(_message); // Emit the event
    }

    // Add a getter function
    function getMessage() external view returns (string memory) {
        return message;
    }

    function setResponse(string calldata _response) external {
        response = _response;
        emit ResponseUpdated(_response);
    }

    function getResponse() external view returns (string memory) {
        return response;
    }
}