import requests
import json

def send_mail(data):
    url = 'https://relay.artisanhosting.net:8000/api/sendmail'
    headers = {'Content-Type': 'application/json'}
    
    try:
        response = requests.post(url, json=data, headers=headers)
        response_data = response.json()

        if response_data['status'] == 'success':
            print(f"Success: {response_data['message']}")
        else:
            print(f"Failed: {response_data.get('message', 'An error occurred.')}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred: {e}")
        print("There was a problem sending your message. Please try again later.")
    except json.JSONDecodeError:
        print("Failed to parse response.")

# Example usage
data = {
    'name': 'John Doe',
    'email': 'john@example.com',
    'message': 'Hello, this is a test message!'
}

send_mail(data)