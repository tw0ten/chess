(async (params) => {
	const e = document.getElementById("board");
	const url = `${
		params.get("s") ||
		"http://localhost:8080"
	}/api`;

	{
		const i = notify("loading wasm");
		await import("./wasm/client.js")
			.then(async (e) => {
				await e.default();
				return notify("INF\nloaded client.wasm");
			}).catch((e) => {
				notify(
					`ERR\n#failed to load client.wasm\n${e}\n...`,
					15000,
					() => {
						document.location.reload();
					},
				);
				throw e;
			}).finally(() => i.remove());
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
		const d = (await get(`${url}/!/${name}`)).split("\n");

		const session = {
			token: d[0],
			board: {
				p: 0,
				m: [],
				d: d[1],
				w: Math.sqrt(d[1].length),
			},
		};

		e.innerHTML = null;
		e.appendChild(((e) => {
			e.innerText = "w";
			e.id = "tl";
			return e;
		})(document.createElement("div")));
		for (let i = 0; i < session.board.w; i++) {
			e.appendChild(((e) => {
				e.id = `${i}:`;
				e.innerText = i;
				return e;
			})(document.createElement("label")));
		}
		for (let i = 0; i < session.board.w; i++) {
			e.appendChild(((e) => {
				e.id = `:${i}`;
				e.innerText = i;
				return e;
			})(document.createElement("label")));
			for (let j = 0; j < session.board.w; j++) {
				e.appendChild(((e) => {
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
				})(document.createElement("button")));
			}
		}

		updateboard(session.board);

		i.remove();
		notify(`joined "${name}"`);
		return session;
	};

	const session = await joingame("game");
	console.log(session.board);

	const h = {
		Authorization: session.token,
	};

	await post(`${url}/!/game`, "", h);
})(new URLSearchParams(document.location.search));
