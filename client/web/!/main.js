(async () => {
	const e = document.getElementById("board");
	const url = `${
		new URLSearchParams(document.location.search).get("s") ||
		"http://localhost:8080"
	}/api`;

	{
		const i = notify("loading wasm");
		await import("../wasm-pack/client.js")
			.then((e) => {
				e.default();
				notify("OK \nloaded client.wasm");
			}).catch((e) => {
				notify(
					`ERR\n#failed to load client.wasm\n#does your browser support it?\n${e}`,
					5000,
				);
			}).finally((_) => {
				i.remove();
			});
	}

	const updateboard = (board) => {
		for (let i = 0; i < board.w; i++) {
			for (let j = 0; j < board.w; j++) {
				document.getElementById(`${i}:${j}`).innerText =
					board.d[i * board.w + j];
			}
		}
	};

	const joingame = async (name) => {
		const i = notify(`joining "${name}"`);
		const d = await get(`${url}/!/${name}`);
		const g = d.split("\n");
		const token = g[0];
		const board = {
			d: g[1],
			w: Math.sqrt(g[1].length),
		};

		e.innerHTML = null;
		e.appendChild((() => {
			const e = document.createElement("div");
			e.innerText = "w";
			e.id = "tl";
			return e;
		})());
		for (let i = 0; i < board.w; i++) {
			e.appendChild((() => {
				const e = document.createElement("label");
				e.id = `${i}:`;
				e.innerText = i;
				return e;
			})());
		}
		for (let i = 0; i < board.w; i++) {
			e.appendChild((() => {
				const e = document.createElement("label");
				e.id = `:${i}`;
				e.innerText = i;
				return e;
			})());
			for (let j = 0; j < board.w; j++) {
				e.appendChild((() => {
					const e = document.createElement("button");
					e.id = `${i}:${j}`;
					const r = document.getElementById(`:${i}`);
					const c = document.getElementById(`${j}:`);
					e.addEventListener("mouseover", () => {
						r.style.color = "var(--acc)";
						c.style.color = "var(--acc)";
					});
					e.addEventListener("mouseout", () => {
						r.style.color = "";
						c.style.color = "";
					});
					return e;
				})());
			}
		}

		updateboard(board);

		notify(`OK \njoined "${name}"`);
		i.remove();
		return { token, board };
	};

	const { token, board } = await joingame("game");

	await post(`${url}/!/game`);
	console.log(token);
	console.log(board);
})();
