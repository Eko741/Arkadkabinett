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

	info_box.innerHTML =
		request.status + " " + request.statusText + "<br>" + request.response;

	if (request.status == "200") color_alert(info_box, "#aaffaa", "#22ee22");
	else color_alert(info_box, "#ffaaaa", "#ff2222");
}

function sendAPIRequest(API, headers) {
	const request = new XMLHttpRequest();
	if (headers != 0)
		for (let i = 0; i < headers.lenght(); i += 2)
			request.setRequestHeader(headers[i], headers[i + 1]);

	request.open("GET", "/API/" + API, true);	

	request.send();

	request.onload = () => {
		handle_response(request);
	};
}


function start() {
	sendAPIRequest("start.api", 0);
}

function stop() {
	sendAPIRequest("stop.api", 0);
}

function test() {
	sendAPIRequest("test.api", 0);
}

function logout() {
	document.cookie =
		"session=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
	location.reload();
}