import {
	GpioEvent,
	Progress,
	WATCH_GPIO_API,
	subscribeToStatus,
	subscribeToProgress,
} from "./data";

export function connect() {
	const socket = new WebSocket("ws://" + window.location.host + WATCH_GPIO_API);
	socket.onopen = () => {
		console.log("socket opend");
	}

	socket.onmessage = (data: any) => {
		const status = JSON.parse(data.data) as GpioEvent;
		subscribeToStatus.set(status.state);
		subscribeToProgress.update((prev: Progress) => { 
			return {
				numberEvents: prev.numberEvents + 1,
				numberIterations: Math.floor(( prev.numberEvents + 1 )/4),
				timeInCycle: status.time,
				latestEvent: status,
			} as Progress
		});
	}

	socket.onclose = (err: any) => {
		console.error(`WS closed with error: ${err.reason}, reconnection in 3s`);
		setTimeout(() => connect(), 3000);
	}

	socket.onerror = (err: any) => {
		console.error(`Socket error: ${err.message}`);
		socket.close(); // start reconnection
	}
}
