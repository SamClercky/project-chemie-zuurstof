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
	"id": "feed" | "delay" | "exhaust",
}
export interface GpioForm {
	all_off: boolean,
	states: GpioEventForm[],
}

export interface GpioInstruction {
	"feed": GpioEvent,
	"delay": GpioEvent,
	"exhaust": GpioEvent,
	"end": GpioEvent,
}

export type ValveState = GpioState[];

export const DEFAULT_DATA_FORM: GpioForm = {
	all_off: true,
	states: [
		{
			id: "feed",
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
			"time": 500,
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
			"time": 3000,
		},
		{
			id: "exhaust",
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
			"time": 10000,
		},
	],
}

export const DEFAULT_VALVE_STATE: GpioState[] = [
	{valve_id: Valve.FEED, status: false},
	{valve_id: Valve.OO, status: false},
	{valve_id: Valve.NN, status: false},
]

export const NEW_INSTRUCTION_API = "/gpio/instruction";
export const WATCH_GPIO_API = "/gpio/ws";

export function sendInstructionToServer(newInstruction: GpioForm) {
	let payload: GpioEvent = {} as GpioEvent;
	let globalTime = 0;
	const off_state: GpioState[] = [
		{valve_id: Valve.FEED, status: false},
		{valve_id: Valve.OO, status: false},
		{valve_id: Valve.NN, status: false},
	];
	
	for (let egpio of newInstruction.states) {
		//@ts-ignore
		payload[egpio.id] = {
			"state": newInstruction.all_off ? off_state : egpio.state,
			"time": globalTime,
		} as GpioEvent;

		// update globalTime
		globalTime += egpio.time; // next events starts after duration
	}
	//@ts-ignore
	payload["end"] = {
			"state": newInstruction.all_off ? off_state : payload["feed"].state,
			"time": globalTime,
	} as GpioEvent;
	console.log(payload);

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
	.then(_ => {
		subscribeToProgress.set({
			numberEvents: 0,
			numberIterations: 0,
			timeInCycle: 0,
			latestEvent: {
				state: DEFAULT_VALVE_STATE,
				time: 0,
			}
		})
	})
	.catch(e => console.error(e));
}

export const subscribeToStatus = writable(DEFAULT_VALVE_STATE);

export interface Progress {
	numberEvents: number,
	numberIterations: number,
	timeInCycle: number,
	latestEvent: GpioEvent,
}
export const subscribeToProgress = writable({
	numberEvents: 0,
	numberIterations: 0,
	timeInCycle: 0,
	latestEvent: {
		state: DEFAULT_VALVE_STATE,
		time: 0,
	},
} as Progress);

// Update every sec the timer in progress
setInterval(() => {
	subscribeToProgress.update(prev => {
		return {
			...prev,
			timeInCycle: prev.timeInCycle + 1000,
		}
	})
}, 1000);
