import json
import requests
data = {}
data_json = json.dumps(data)
payload = {'json_payload': data_json}
#for i in range(10):
r = requests.get('http://192.168.1.5:8000/query', data=payload)
print r.status_code
#print r.text
#print r.json()

#data_out = r.json()
data_out = r.text
loadout = {'json_payload': loadout}
q = requests.get('http://192.168.1.4:8008', data=loadout)
print q.status_code
