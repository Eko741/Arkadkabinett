text/javascript

const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay));
var publicKeyPEM;
var publicKey = "";

window.addEventListener('load', function () {
  pki = forge.pki;
})

async function  color_alert(element, color_start, color_end) {
  element.style.transitionDuration = "0s";
  element.style.backgroundColor = color_start;
  await sleep(10);
  element.style.transitionDuration = "0.2s";
  element.style.backgroundColor = color_end;
}

function handle_response(request) {
  const info_box = document.getElementById("info_box");

  info_box.innerHTML = request.status + " " + request.statusText + "<br>" + request.response;
  
  
  if (request.status == "200")
    color_alert(info_box, "#aaffaa", "#22ee22");
  else
    color_alert(info_box, "#ffaaaa", "#ff2222");
}

function sendAPIRequest(API, key){
  const request = new XMLHttpRequest();
  request.open('GET', 'API/' + API, true);

  //startRequest.setRequestHeader("Key", document.getElementById("key").value);
  request.setRequestHeader("Key_Encrypted", encrypt_data(key));

  request.send();
  
  request.onload = ()=> {
    handle_response(request);
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