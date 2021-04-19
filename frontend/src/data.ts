//@ts-ignore
import {writable} from "svelte/store";

export enum Valve {
	FEED = "FEED",
	OO = "OO",
	NN = "NN",
}

export interface GpioState {
	"valve_id": Valve,
	"status": boolean,
}

export interface GpioEvent {
	"state": GpioState[],
	"time": number,
}

export interface GpioEventForm extends GpioEvent {
	"id": "feed" | "delay" | "exhaust" | "end",
}

export interface GpioInstruction {
	"feed": GpioEvent,
	"delay": GpioEvent,
	"exhaust": GpioEvent,
	"end": GpioEvent,
}

export type ValveState = GpioState[];

export const DEFAULT_DATA_FORM: GpioEventForm[] = [
	{
		id: "feed",
		"state": [
			{
				"valve_id": Valve.FEED,
				"status": true,
			},
			{
				"valve_id": Valve.OO,
				"status": false,
			},
			{
				"valve_id": Valve.NN,
				"status": false,
			}
		],
		"time": 60,
	},
	{
		id: "delay",
		"state": [
			{
				"valve_id": Valve.FEED,
				"status": false,
			},
			{
				"valve_id": Valve.OO,
				"status": false,
			},
			{
				"valve_id": Valve.NN,
				"status": false,
			}
		],
		"time": 280,
	},
	{
		id: "exhaust",
		"state": [
			{
				"valve_id": Valve.FEED,
				"status": true,
			},
			{
				"valve_id": Valve.OO,
				"status": false,
			},
			{
				"valve_id": Valve.NN,
				"status": false,
			}
		],
		"time": 280,
	},
	{
		id: "end",
		"state": [
			{
				"valve_id": Valve.FEED,
				"status": true,
			},
			{
				"valve_id": Valve.OO,
				"status": false,
			},
			{
				"valve_id": Valve.NN,
				"status": false,
			}
		],
		"time": 280,
	}
];

export const DEFAULT_VALVE_STATE: GpioState[] = [
	{valve_id: Valve.FEED, status: false},
	{valve_id: Valve.OO, status: false},
	{valve_id: Valve.NN, status: false},
]

export const NEW_INSTRUCTION_API = "/gpio/instruction";
export const WATCH_GPIO_API = "/gpio/ws"

export function sendInstructionToServer(newInstruction: GpioEventForm[]) {
	let payload: GpioEvent = {} as GpioEvent;
	for (let egpio of newInstruction) {
		console.log(egpio);
		//@ts-ignore
		payload[egpio.id] = {
			"state": egpio.state,
			"time": egpio.time,
		} as GpioEvent;
	}

	// send payload to server
	fetch(NEW_INSTRUCTION_API, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		redirect: "follow",
		body: JSON.stringify(payload),
	})
	.then(resp => console.log(`Gpio server says: ${resp.status}`))
	.catch(e => console.error(e));
}

const socket = new WebSocket("ws://" + window.location.host + WATCH_GPIO_API);

export const subscribeToStatus = (() => {
	const store = writable(DEFAULT_VALVE_STATE);

	socket.onopen = () => {
		console.log("socket opend");
	}

	socket.onmessage = (data: any) => {
		const status = JSON.parse(data.data) as ValveState;
		store.set(status);
	}

	return store;
})();

