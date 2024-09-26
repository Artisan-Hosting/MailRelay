require 'net/http'
require 'json'
require 'uri'

def send_mail(data)
  url = URI.parse('https://relay.artisanhosting.net:8000/api/sendmail')
  http = Net::HTTP.new(url.host, url.port)
  http.use_ssl = true
  
  headers = { 'Content-Type' => 'application/json' }
  request = Net::HTTP::Post.new(url.path, headers)
  request.body = data.to_json

  begin
    response = http.request(request)
    response_data = JSON.parse(response.body)

    if response_data['status'] == 'success'
      puts "Success: #{response_data['message']}"
    else
      puts "Failed: #{response_data['message'] || 'An error occurred.'}"
    end
  rescue StandardError => e
    puts "An error occurred: #{e.message}"
    puts "There was a problem sending your message. Please try again later."
  end
end

# Example usage
data = {
  name: 'John Doe',
  email: 'john@example.com',
  message: 'Hello, this is a test message!'
}

send_mail(data)
