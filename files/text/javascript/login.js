const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay));

async function color_alert(element, color_start, color_end) {
	element.style.transitionDuration = "0s";
	element.style.backgroundColor = color_start;
	await sleep(10);
	element.style.transitionDuration = "0.2s";
	element.style.backgroundColor = color_end;
}

function handle_response(request) {
	const info_box = document.getElementById("info_box");

    if (request.status == "200") 
        color_alert(info_box, "#aaffaa", "#22ee22");
	else if (request.status == "204"){
        location.replace(request.getResponseHeader("Redirect"));
        return;
    }
    else  
        color_alert(info_box, "#ffaaaa", "#ff2222");

	info_box.innerHTML =
		request.status + " " + request.statusText + "<br>" + request.response;
}

function sendAPIRequest(API, headers) {
    const request = new XMLHttpRequest();
    request.open("GET", "/API/" + API, true);

	if (headers != 0)
		for (let i = 0; i < headers.length; i += 2)
			request.setRequestHeader(headers[i], headers[i + 1]);	

	request.send();

	request.onload = () => {
		handle_response(request);
	};
}


function login() {
	var password = document.getElementById("password").value;

	// get current time in seconds since the unix epoch
	var currentTime = Date.now();

	hash(password + currentTime).then((hash) => {
		document.cookie = "session=" + hash + "; path=/;";
		document.cookie = "session-created=" + currentTime + "; path=/;";
		sendAPIRequest("login.api", 0);
	});
}

async function hash(string) {
	const utf8 = new TextEncoder().encode(string);
	const hashBuffer = await crypto.subtle.digest("SHA-256", utf8);

	const hashArray = Array.from(new Uint8Array(hashBuffer));
	const hashHex = hashArray
		.map((bytes) => bytes.toString(16).padStart(2, "0"))
		.join("");
	return hashHex;
}
