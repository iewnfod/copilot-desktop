import {invoke} from "@tauri-apps/api/core";
import {
	Box,
	Card,
	Divider,
	Stack,
	Switch,
	Tab,
	TabList,
	TabPanel,
	Tabs,
	Typography,
	TypographySystem
} from "@mui/joy";
import { Preference } from "./preference";

export default function SettingsPage({pre, setPre} : {pre: Preference, setPre: (pre: Preference) => void}) {
	function updatePreference(preKey: keyof Preference, value: any) {
		const newPreference: Preference = JSON.parse(JSON.stringify(pre));
		// @ts-ignore
		newPreference[preKey] = value;
		invoke("set_preference", {pre: newPreference}).then(() => {
			setPre(newPreference);
		});
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
