import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

export default function AIPage() {
	useEffect(() => {
		invoke("set_window");

		invoke("get_ai_url").then((url) => {
			if (typeof url === "string" && url) {
				window.location.replace(url);
			}
		})
	}, []);

	return (
		<></>
	)
}
