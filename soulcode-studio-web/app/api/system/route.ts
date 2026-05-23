import { NextResponse } from 'next/server';

export async function GET() {
  const fakeData = {
    traffic: (Math.random() * 2).toFixed(1),
    blocked: 14281,
    lastIp: "",
    
    bannedList: [
      { ip: "", reason: "SSH Brute Force", time: "18:02" },
      { ip: "", reason: "XDP Drop", time: "18:05" },
      { ip: "", reason: "Labyrinth", time: "18:07" }
    ]
  };

  return NextResponse.json(fakeData);
}