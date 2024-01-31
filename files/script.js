text/javascript
var publicKeyPEM;
var publicKey = "";

window.addEventListener('load', function () {
  pki = forge.pki;
})

function displayInfo(){

}

function sendAPIRequest(API, key){
  const request = new XMLHttpRequest();
  request.open('GET', 'API/' + API, true);

  //startRequest.setRequestHeader("Key", document.getElementById("key").value);
  request.setRequestHeader("Key_Encrypted", encrypt_data(key));

  request.send();
  
  request.onload = ()=> {
    alert(request.status + " " + request.statusText + " \r\n" + request.response);
  };
}


function start(){
  sendAPIRequest('start', document.getElementById("key").value);
}

function stop(){
  sendAPIRequest('stop', document.getElementById("key").value);
}

function test(){
  sendAPIRequest('test', document.getElementById("key").value);
}


function encrypt_data(data){
  if (publicKey == ""){
    const pubKeyRequest = new XMLHttpRequest();
  
    pubKeyRequest.open('GET', 'API/RSA_Key', false);
    
    pubKeyRequest.send();
    
    publicKeyPEM = pubKeyRequest.response;
    publicKey = pki.publicKeyFromPem(publicKeyPEM);
  }

  var encrypted = publicKey.encrypt(data, 'RSA-OAEP', {
    md: forge.md.sha256.create()
  });

  encrypted = forge.util.encode64(encrypted);
  
  console.log(encrypted);

  return encrypted;
}