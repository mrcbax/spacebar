#import socket module
from socket import *
serverPort = 8008
serverSocket = socket(AF_INET, SOCK_STREAM)
#Prepare a sever socket
serverSocket.bind(('', serverPort))
serverSocket.listen(1)
while True:
    #Establish the connection
    print 'Ready to serve...'
    connectionSocket, addr = serverSocket.accept()
    try:
        message = connectionSocket.recv(1024)
        print message
        #Send one HTTP header line into socket
        connectionSocket.send('HTTP/1.0 200 OK')
    except IOError:
        #Send response message for file not found
        connectionSocket.send('HTTP/1.0 404 Not Found')
        print 'File not found'
    finally:
        #Close client socket
        connectionSocket.close()
serverSocket.close()
