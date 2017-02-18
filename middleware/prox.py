from socket import *

inPort = 8000
inSocket = socket(AF_INET, SOCK_STREAM)
inSocket.bind(('', inPort))
inSocket.listen(1)

outName = '192.168.1.5'
outPort = 8008
outSocket = socket(AF_INET, SOCK_STREAM)

while True:
    print 'Ready to serve...'
    connectionSocket, addr = inSocket.accept()
    try:
        message = connectionSocket.recv(1024)
        print message
        outSocket.connect((outName, outPort))
        outSocket.send(message)
        connectionSocket.send('HTTP/1.0 200 OK')
        temp = outSocket.recv(1024)
        print 'From Server:'
        while temp != '':
            print temp
            temp = outSocket.recv(1024)
        outSocket.close()
    except IOError:
        connectionSocket.send('HTTP/1.0 404 Not Found')
        print 'File not found'
    finally:
        connectionSocket.close()
inSocket.close()
