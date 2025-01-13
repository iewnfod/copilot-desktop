import {invoke} from "@tauri-apps/api/core";
import {
	Box, Button,
	Card,
	Divider, Input,
	Stack,
	Switch,
	Tab,
	TabList,
	TabPanel,
	Tabs,
	Typography,
	TypographySystem
} from "@mui/joy";
import {DEFAULT_PREFERENCE, Preference} from "./preference";
import {useEffect, useState} from "react";
import toast from "react-hot-toast";

export default function SettingsPage() {
	const [pre, setPre] = useState<Preference>(DEFAULT_PREFERENCE);
	const [AIUrlInput, setAIUrlInput] = useState(pre.ai_url);

	useEffect(() => {
		invoke("get_preference").then((pre) => {
			// @ts-ignore
			setPre(pre ?? DEFAULT_PREFERENCE);
		});
	}, []);

	useEffect(() => {
		setAIUrlInput(pre.ai_url);
	}, [pre, setPre]);

	function updatePreference(preKey: keyof Preference, value: any) {
		const newPreference: Preference = JSON.parse(JSON.stringify(pre));
		// @ts-ignore
		newPreference[preKey] = value;
		invoke("set_preference", {pre: newPreference}).then(() => {
			setPre(newPreference);
			toast.success("Preference updated successfully.", {id: "pus"});
		});
	}

	function submitAIUrl() {
		let url: string = AIUrlInput ?? pre.ai_url;
		updatePreference("ai_url", url);
	}

	return (
		<Tabs
			orientation="vertical"
			variant="outlined"
			size="lg"
			sx={{width: "100%", height: "100%", userSelect: "None"}}
		>
			<TabList>
				<Tab>
					<Typography level="title-md">
						General
					</Typography>
				</Tab>
				<Tab>
					<Typography level="title-md">
						About
					</Typography>
				</Tab>
			</TabList>
			<TabPanel value={0}>
				<Box sx={{display: 'flex', flexDirection: 'column', gap: 2, width: '100%', height: '100%', overflowY: 'auto'}}>
					<Card variant="soft">
						<Box sx={{display: 'flex', flexDirection: 'column', justifyContent: 'flex-start', alignItems: 'center', gap: 2, p: 1}}>
							<Stack direction="row" justifyContent="space-between" sx={{width: '100%'}}>
								<Typography level="title-md">
									Start at login
								</Typography>
								<Switch
									checked={pre.start_at_login}
									onChange={() => updatePreference("start_at_login", !pre.start_at_login)}
								/>
							</Stack>
							<Divider orientation="horizontal" sx={{width: '100%'}}/>
							<Stack direction="row" justifyContent="space-between" sx={{width: '100%'}}>
								<Stack>
									<Typography level="title-md">
										Hide window when focus lost
									</Typography>
									<RestartSentence/>
								</Stack>
								<Switch
									checked={pre.hide_window_when_not_focus}
									onChange={() => updatePreference("hide_window_when_not_focus", !pre.hide_window_when_not_focus)}
								/>
							</Stack>
							<Divider orientation="horizontal" sx={{width: '100%'}}/>
							<Stack direction="row" justifyContent="space-between" sx={{width: '100%'}}>
								<Stack>
									<Typography level="title-md">
										Always on top
									</Typography>
									<RestartSentence/>
								</Stack>
								<Switch
									disabled={pre.hide_window_when_not_focus}
									checked={pre.always_on_top}
									onChange={() => updatePreference("always_on_top", !pre.always_on_top)}
								/>
							</Stack>
						</Box>
					</Card>
					<Card variant="soft">
						<Box sx={{display: 'flex', flexDirection: 'column', justifyContent: 'flex-start', alignItems: 'center', gap: 2, p: 1}}>
							<Stack sx={{width: '100%'}} gap={1}>
								<Stack>
									<Typography level="title-md">
										Copilot Site
									</Typography>
									<RestartSentence/>
								</Stack>
								<Stack direction="row" justifyContent="space-between" sx={{width: '100%'}} gap={1}>
									<Input
										value={AIUrlInput}
										sx={{flexGrow: 1}}
										id="ai-url-input"
										endDecorator={(
											<Button
												sx={{mt: 1, mb: 1, mr: 0}}
												onClick={() => submitAIUrl()}
											>
												Save
											</Button>
										)}
										onChange={(e) => setAIUrlInput(e.target.value)}
									/>
								</Stack>
							</Stack>
						</Box>
					</Card>
				</Box>
			</TabPanel>
		</Tabs>
	);
}

function RestartSentence({level} : {level?: keyof TypographySystem}) {
	return (
		<Typography level={level ?? "body-xs"}>
			You may need restart this app to activate this config.
		</Typography>
	);
}
