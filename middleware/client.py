from socket import *
import sys
import argparse

parser = argparse.ArgumentParser(description='Connect to remote servers')

parser.add_argument('server', metavar='IP', 
                    help='remote server to connect to')
parser.add_argument('port', metavar='PORT', type=int,
                    help='port to connect to')
#parser.add_argument('file', metavar='FILE', 
#                    help='file to retrieve')
args = parser.parse_args()

serverName = args.server
serverPort = args.port
#filename = args.file
clientSocket = socket(AF_INET, SOCK_STREAM)
clientSocket.connect((serverName, serverPort))

#print 'sending'
#f = open('/home/justin/blank.txt', 'r')
#for line in f:
#    clientSocket.send(line)
#print 'file sent'



temp = clientSocket.recv(1024)
print 'From Server:'
while temp != '':
    print temp
    temp = clientSocket.recv(1024)
clientSocket.close()
