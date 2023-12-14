



function start(){
  const xhttpr = new XMLHttpRequest();
  xhttpr.open('GET', 'API/start', true);
  
  xhttpr.send();
  
  xhttpr.onload = ()=> {
    if (xhttpr.status === 200) {
    
      alert(xhttpr.response);
    } else {
      // Handle error
    }
  };
}

function stop(){
  const xhttpr = new XMLHttpRequest();
  xhttpr.open('GET', 'API/stop', true);
    
  xhttpr.send();
    
  xhttpr.onload = ()=> {
    if (xhttpr.status === 200) {
      
        alert(xhttpr.response);
    } else {
        // Handle error
    }
  };
}
