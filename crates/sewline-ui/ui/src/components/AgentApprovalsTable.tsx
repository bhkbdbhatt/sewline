import React from 'react';
import { ApprovalRequest } from '../types';
import { Bot, UserCheck, ShieldAlert, Check, X } from 'lucide-react';

interface Props {
    approvals: ApprovalRequest[];
    onResolve: (approvalId: string, approve: boolean) => void;
}

export const AgentApprovalsTable: React.FC<Props> = ({ approvals, onResolve }) => {
    const getRiskBadge = (risk: ApprovalRequest['riskLevel']) => {
        switch (risk) {
            case 'CRITICAL':
                return 'bg-rose-950 text-rose-300 border-rose-800';
            case 'HIGH':
                return 'bg-amber-950 text-amber-300 border-amber-800';
            case 'MEDIUM':
                return 'bg-yellow-950 text-yellow-300 border-yellow-800';
            default:
                return 'bg-slate-800 text-slate-300 border-slate-700';
        }
    };

    return (
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 shadow-xl">
            <h3 className="text-lg font-bold text-slate-100 mb-4 flex items-center gap-2">
                <UserCheck className="w-5 h-5 text-indigo-400" />
                Agent Human-in-the-Loop Approval Queue
            </h3>

            <div className="overflow-x-auto">
                <table className="w-full text-left text-sm text-slate-300">
                    <thead className="bg-slate-950 text-xs text-slate-400 uppercase">
                        <tr>
                            <th className="p-3">Agent</th>
                            <th className="p-3">Action Description</th>
                            <th className="p-3">Risk Level</th>
                            <th className="p-3">Status</th>
                            <th className="p-3 text-right">Decision</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y divide-slate-800/60">
                        {approvals.map((req) => (
                            <tr key={req.approvalId} className="hover:bg-slate-850/50">
                                <td className="p-3 font-mono text-xs text-indigo-300 flex items-center gap-2">
                                    <Bot className="w-4 h-4 text-indigo-400 shrink-0" />
                                    {req.agentId}
                                </td>
                                <td className="p-3">{req.actionDescription}</td>
                                <td className="p-3">
                                    <span className={`px-2 py-0.5 rounded border text-[10px] font-bold ${getRiskBadge(req.riskLevel)}`}>
                                        {req.riskLevel}
                                    </span>
                                </td>
                                <td className="p-3 font-medium text-xs">{req.status}</td>
                                <td className="p-3 text-right">
                                    {req.status === 'PENDING' ? (
                                        <div className="flex items-center justify-end gap-2">
                                            <button
                                                onClick={() => onResolve(req.approvalId, true)}
                                                className="p-1.5 rounded bg-emerald-950 hover:bg-emerald-900 border border-emerald-800 text-emerald-400"
                                                title="Approve Action"
                                            >
                                                <Check className="w-4 h-4" />
                                            </button>
                                            <button
                                                onClick={() => onResolve(req.approvalId, false)}
                                                className="p-1.5 rounded bg-rose-950 hover:bg-rose-900 border border-rose-800 text-rose-400"
                                                title="Reject Action"
                                            >
                                                <X className="w-4 h-4" />
                                            </button>
                                        </div>
                                    ) : (
                                        <span className="text-xs text-slate-500 italic">Resolved</span>
                                    )}
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
};