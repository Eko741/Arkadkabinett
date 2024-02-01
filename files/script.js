text/javascript;
var publicKeyPEM;
var publicKey = "";

window.addEventListener("load", function () {
	pki = forge.pki;
});

function displayInfo() {}

function sendAPIRequest(API, key) {
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
	sendAPIRequest("start", document.getElementById("key").value);
}

function stop() {
	sendAPIRequest("stop", document.getElementById("key").value);
}

function test() {
	sendAPIRequest("test", document.getElementById("key").value);
}

function logout() {
	document.cookie =
		"session=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
	location.reload();
}

function encrypt_data(data) {
	if (publicKey == "") {
		const pubKeyRequest = new XMLHttpRequest();

		pubKeyRequest.open("GET", "API/RSA_Key", false);

		pubKeyRequest.send();

		publicKeyPEM = pubKeyRequest.response;
		publicKey = pki.publicKeyFromPem(publicKeyPEM);
	}

	var encrypted = publicKey.encrypt(data, "RSA-OAEP", {
		md: forge.md.sha256.create(),
	});

	encrypted = forge.util.encode64(encrypted);

	console.log(encrypted);

	return encrypted;
}
