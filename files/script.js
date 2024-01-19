function start(){
  const xhttpr = new XMLHttpRequest();
  xhttpr.open('GET', 'API/start', true);

  xhttpr.setRequestHeader("Key", document.getElementById("key").value);
  
  xhttpr.send();
  
  xhttpr.onload = ()=> {
    alert(xhttpr.status + " " + xhttpr.statusText + " \r\n" + xhttpr.response);
  };
}

function stop(){
  const xhttpr = new XMLHttpRequest();
  xhttpr.open('GET', 'API/stop', true);

  xhttpr.setRequestHeader("Key", document.getElementById("key").value);
    
  xhttpr.send();
    
  xhttpr.onload = ()=> {
    alert(xhttpr.status + " " + xhttpr.statusText + " \r\n" + xhttpr.response);
  };
}