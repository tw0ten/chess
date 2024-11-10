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

	const game = "gametiltlte";

	await import("../wasm-pack/client.js").then((m) => m.default());
	const g = (await get(`!/${game}`)).split("\n");
	const token = g[0];
	const b = g[1];
	for (let i = 0; i < Math.sqrt(b.length); i++) {
		for (let j = 0; j < Math.sqrt(b.length); j++) {
			document.getElementById(`${i}:${j}`).innerText =
				b[i * Math.sqrt(b.length) + j];
		}
	}
	console.log(g);
	const l = await post(`!/${game}`, { "Authorization": token }, "3:6-3:4");
	console.log(l);
})();
