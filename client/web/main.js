(() => {
	const url = `${
		new URLSearchParams(document.location.search).get("s") || ""
	}/api/!`;

	console.log(`server: ${url}`);

	const get = async (i) => {
		const response = await fetch(`${url}/${i}`);
		if (!response.ok) return;
		return await response.text();
	};

	const post = async (i, h, b) => {
		const response = await fetch(`${url}/${i}`, {
			method: "POST",
			headers: JSON.parse(h),
			body: b,
		});
		if (!response.ok) return;
		return await response.text();
	};

	import("./wasm-pack/client.js").then((m) => m.default());
})();
