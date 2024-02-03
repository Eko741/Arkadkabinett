const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay));
var publicKeyPEM;
var publicKey = "";

window.addEventListener("load", function () {
	const urlParams = new URLSearchParams(window.location.search);
	const errorParam = urlParams.get("error");
	if (errorParam) {
		document.getElementById("error").innerHTML = "Error: " + errorParam;
	}

	pki = forge.pki;
});

async function color_alert(element, color_start, color_end) {
	element.style.transitionDuration = "0s";
	element.style.backgroundColor = color_start;
	await sleep(10);
	element.style.transitionDuration = "0.2s";
	element.style.backgroundColor = color_end;
}

function handle_response(request) {
	const info_box = document.getElementById("info_box");

	info_box.innerHTML =
		request.status + " " + request.statusText + "<br>" + request.response;

	if (request.status == "200") color_alert(info_box, "#aaffaa", "#22ee22");
	else color_alert(info_box, "#ffaaaa", "#ff2222");
}

function sendAPIRequest(API) {
	const request = new XMLHttpRequest();
	request.open("GET", "API/" + API, true);

	request.send();

	request.onload = () => {
		handle_response(request);
	};
}

function displayInfo() {}

function sendAPIRequest(API, password) {
	const request = new XMLHttpRequest();
	request.open("GET", "API/" + API, true);
	request.send();
	request.onload = () => {
		alert(
			request.status +
				" " +
				request.statusText +
				" \r\n" +
				request.response
		);
	};
}

function start() {
	sendAPIRequest("start");
}

function stop() {
	sendAPIRequest("stop");
}

function test() {
	sendAPIRequest("test");
}

function login() {
	var password = document.getElementById("password").value;

	// get current time in seconds since the unix epoch
	var currentTime = Date.now();

	hash(password + currentTime).then((hash) => {
		document.cookie = "session=" + hash + "; path=/;";
		document.cookie = "session-created=" + currentTime + "; path=/;";
		location.replace("/admin");
	});
}

function logout() {
	document.cookie =
		"session=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
	location.reload();
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
