"use client";
import React, { useState, useEffect, useCallback } from 'react';
import { LineChart, Line, ResponsiveContainer } from 'recharts';


const AUTH_KEY = "HelgaSoulcode";
type Lang = 'ua' | 'en';

const TRANSLATIONS = {
  ua: { title: "SENTINEL", subtitle: "SYSTEM", desc: "Автономна інфраструктура захисту на базі eBPF. Гібридний стек (Rust/Go/Elixir) забезпечує фільтрацію трафіку з нульовою затримкою.", traffic: "ТРАФІК ВУЗЛА", cpu: "CPU LOAD", banned: "ЗАБЛОКОВАНІ", traps: "МОНІТОРИНГ ПАСТОК", map: "ГЕОГРАФІЯ: UA_NODE_ALPHA_01", active: "СИСТЕМА АКТИВНА", admin: "KERNEL_ROOT_TERMINAL", block: "БЛОКУВАТИ", exit: "ВИХІД", time: "ЧАС", violation: "ПОРУШЕННЯ", captures: "ЗАХОПЛЕНО" },
  en: { title: "SENTINEL", subtitle: "SYSTEM", desc: "Autonomous eBPF-based defense infrastructure. Hybrid stack (Rust/Go/Elixir) enables zero-latency packet filtering.", traffic: "NODE TRAFFIC", cpu: "CPU LOAD", banned: "BANNED ENTITIES", traps: "TRAP MONITORING", map: "NODE GEOGRAPHY: UA_NODE_ALPHA_01", active: "SYSTEM ACTIVE", admin: "KERNEL_ROOT_TERMINAL", block: "BLOCK", exit: "EXIT", time: "TIMESTAMP", violation: "VIOLATION", captures: "CAPTURES" }
};


const AdminTerminal = ({ t, onClose, onBlock }: any) => (
  <div className="max-w-[1700px] mx-auto p-12 border border-red-900/20 bg-[#080808] relative shadow-2xl animate-in fade-in duration-700">
    <div className="flex justify-between items-center mb-12 border-b border-red-900/20 pb-8">
      <span className="text-xs font-mono text-red-600 tracking-[0.5em] font-black uppercase">{t.admin}</span>
      <button onClick={onClose} className="px-8 py-3 bg-red-950/20 border border-red-900 text-[10px] text-red-600 hover:bg-red-600 hover:text-white uppercase transition-all">[ {t.exit} ]</button>
    </div>
    <div className="grid grid-cols-1 md:grid-cols-4 gap-16">
      <div className="space-y-4">
        <div className="p-4 border border-zinc-900 bg-black text-[9px] font-mono text-zinc-500 uppercase">{`Integrity: SECURE // Node: UA_01`}</div>
        <button className="w-full text-left p-5 border border-zinc-800 text-[10px] hover:border-red-600 uppercase font-black transition-all">_FLUSH_XDP_RULES</button>
      </div>
      <div className="md:col-span-3">
        <table className="w-full text-[10px] uppercase font-mono">
          <thead className="text-zinc-700 text-left">
            <tr><th className="pb-4">{t.time}</th><th className="pb-4">IPV4_ORIGIN</th><th className="pb-4">{t.violation}</th><th className="pb-4 text-right">ACTION</th></tr>
          </thead>
          <tbody className="text-zinc-500">
            {['103.203.57.122', '45.129.33.18', '185.12.44.1'].map((ip, i) => (
              <tr key={i} className="border-b border-zinc-900 hover:bg-zinc-900/50">
                <td className="py-5">12:0{i}:00</td><td className="text-white font-bold">{ip}</td><td className="text-red-900">GHOST_NET_TRAP</td>
                <td className="text-right"><button onClick={onBlock} className="border border-red-900 px-4 py-1 hover:bg-red-600 hover:text-white transition-colors">[ {t.block} ]</button></td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  </div>
);


export default function Sentinel() {
  const [lang, setLang] = useState<Lang>('ua');
  const [showAdmin, setShowAdmin] = useState(false);
  const [stats, setStats] = useState({ traffic: "1.8", blocked: 14282, cpu: 12, chartData: Array.from({ length: 20 }, () => ({ val: Math.random() * 20 })) });

  const t = TRANSLATIONS[lang];

  useEffect(() => {
    const handleKeys = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.shiftKey && e.key === 'S') {
        if (prompt("Enter Access Key:") === AUTH_KEY) setShowAdmin(true);
      }
    };
    window.addEventListener('keydown', handleKeys);
    return () => window.removeEventListener('keydown', handleKeys);
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      setStats(prev => ({
        ...prev,
        traffic: (Math.random() * 1.4 + 0.6).toFixed(1),
        cpu: Math.floor(Math.random() * 8 + 8),
        chartData: [...prev.chartData.slice(1), { val: Math.random() * 25 }]
      }));
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="min-h-screen bg-[#050505] text-white p-6 lg:p-12 font-sans overflow-x-hidden">
      {/* Header and main layout here using components above */}
      <header className="flex justify-between items-start mb-24 border-b border-white/5 pb-8">
        <div>
          <span className="text-[10px] font-black tracking-[0.5em] text-zinc-500 uppercase">SoulCode Studio</span>
          <h1 className="text-xl font-bold uppercase"></h1>
        </div>
        <div className="flex gap-2">
            {(['ua', 'en'] as Lang[]).map(l => (
                <button key={l} onClick={() => setLang(l)} className={`px-4 py-1 text-[10px] font-mono border ${lang === l ? 'bg-white text-black' : 'border-zinc-800'}`}>{l.toUpperCase()}</button>
            ))}
        </div>
      </header>
      
      {/* Metrics Row */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-px bg-zinc-900 border border-zinc-900 mb-20 max-w-[1700px] mx-auto">
        {[ { label: t.traffic, val: stats.traffic, unit: 'GBPS' }, { label: t.banned, val: stats.blocked, unit: 'COUNT' }, { label: t.cpu, val: `${stats.cpu}%`, unit: 'LOAD' } ].map((item, i) => (
          <div key={i} className="bg-[#050505] p-12 h-64 flex flex-col justify-between">
            <span className="text-[10px] text-zinc-500 tracking-widest uppercase">{item.label}</span>
            <div className="text-7xl font-mono tracking-tighter">{item.val}</div>
          </div>
        ))}
      </div>

      {showAdmin && <AdminTerminal t={t} onClose={() => setShowAdmin(false)} onBlock={() => setStats(s => ({...s, blocked: s.blocked + 1}))} />}
    </div>
  );
}