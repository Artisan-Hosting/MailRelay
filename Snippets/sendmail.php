<?php

function sendMail($data)
{
    // Define the API endpoint URL
    $url = 'https://relay.artisanhosting.net:8000/api/sendmail';

    // Initialize a cURL session
    $ch = curl_init($url);

    // Convert data array to JSON format
    $payload = json_encode($data);

    // Set cURL options for a POST request
    curl_setopt($ch, CURLOPT_POST, true);
    curl_setopt($ch, CURLOPT_HTTPHEADER, array('Content-Type: application/json'));
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($ch, CURLOPT_POSTFIELDS, $payload);

    // Execute the POST request and store the response
    $response = curl_exec($ch);

    // Check for cURL errors
    if (curl_errno($ch)) {
        // Handle cURL error
        echo 'Curl error: ' . curl_error($ch);
        return [
            'status' => 'failure',
            'message' => 'There was a problem sending your message. Please try again later.'
        ];
    }

    // Close the cURL session
    curl_close($ch);

    // Parse the response data if it's a string
    $responseData = json_decode($response, true);
    
    // Check if decoding JSON failed
    if (json_last_error() !== JSON_ERROR_NONE) {
        // Handle JSON parsing error
        return [
            'status' => 'failure',
            'message' => 'Failed to parse response.'
        ];
    }

    // Handle the response
    if ($responseData['status'] === 'success') {
        // Clear input or do further action on success
        return [
            'status' => 'success',
            'message' => $responseData['message']
        ];
    } else {
        // Handle failure response from the API
        return [
            'status' => 'failure',
            'message' => $responseData['message'] ?? 'An error occurred.'
        ];
    }
}

// Example usage of the function
$data = [
    'name' => 'John Doe',
    'email' => 'john@example.com',
    'message' => 'Hello, this is a test message!'
];

$response = sendMail($data);

if ($response['status'] === 'success') {
    echo "Success: " . $response['message'];
} else {
    echo "Failed: " . $response['message'];
}

?>
