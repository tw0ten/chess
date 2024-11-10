(async () => {
	const url = `${
		new URLSearchParams(document.location.search).get("s") || ""
	}/api`;

	console.log(`server: ${url}`);

	const get = async (i) => {
		const response = await fetch(`${url}/${i}`);
		if (!response.ok) return;
		return await response.text();
	};

	const post = async (i, h, b) => {
		const response = await fetch(`${url}/${i}`, {
			method: "POST",
			headers: h,
			body: b,
		});
		if (!response.ok) return;
		return await response.text();
	};

	await import("./wasm-pack/client.js").then((m) => m.default());
	const g = (await get("!/LULE")).split("\n")[0].split(":")[1];
	console.log(g);
	const l = await post("!/LULE", { "Authorization": g }, "HIIIIIII oWo");
	console.log(l);
})();
