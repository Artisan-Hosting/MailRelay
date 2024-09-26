const axios = require('axios');

async function sendMail(data) {
    const url = 'https://relay.artisanhosting.net:8000/api/sendmail';
    const headers = {
        'Content-Type': 'application/json'
    };

    try {
        const response = await axios.post(url, data, { headers });
        let responseData = response.data;

        if (typeof responseData === 'string') {
            responseData = JSON.parse(responseData);
        }

        if (responseData.status === 'success') {
            console.log("Success: " + responseData.message);
        } else {
            console.log("Failed: " + (responseData.message || 'An error occurred.'));
        }
    } catch (error) {
        console.error('An error occurred: ', error.message);
        console.log('There was a problem sending your message. Please try again later.');
    }
}

// Example usage
const data = {
    name: 'John Doe',
    email: 'john@example.com',
    message: 'Hello, this is a test message!'
};

sendMail(data);
