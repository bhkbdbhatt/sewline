import React from 'react';
import { Stage } from '../types';
import { ShieldCheck, ShieldAlert, FileKey, Download } from 'lucide-react';

interface Props {
    stage?: Stage;
    onExportEvidence: (gateId: string) => void;
}

export const GateStatusCard: React.FC<Props> = ({ stage, onExportEvidence }) => {
    if (!stage) {
        return (
            <div className="bg-slate-900 border border-slate-800 rounded-xl p-8 text-center text-slate-500">
                Select a stage from the pipeline flow above to inspect policy gates and evidence attestations.
            </div>
        );
    }

    return (
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 shadow-xl">
            <h3 className="text-lg font-bold text-slate-100 mb-4 flex items-center gap-2">
                Stage Gates & Attestations: <span className="text-indigo-400">{stage.name}</span>
            </h3>

            {stage.gates.length === 0 ? (
                <p className="text-sm text-slate-500 italic">No compliance gates enforced on this stage.</p>
            ) : (
                <div className="space-y-3">
                    {stage.gates.map((gate) => (
                        <div
                            key={gate.id}
                            className="p-4 rounded-lg bg-slate-950 border border-slate-800 flex items-center justify-between"
                        >
                            <div className="flex items-start gap-3">
                                {gate.passed ? (
                                    <ShieldCheck className="w-5 h-5 text-emerald-400 shrink-0 mt-0.5" />
                                ) : (
                                    <ShieldAlert className="w-5 h-5 text-rose-500 shrink-0 mt-0.5" />
                                )}
                                <div>
                                    <h4 className="text-sm font-semibold text-slate-200">{gate.name}</h4>
                                    <p className="text-xs font-mono text-slate-400 mt-0.5">Rule: {gate.policyRule}</p>
                                    {gate.attestationHash && (
                                        <p className="text-[11px] font-mono text-indigo-400 mt-1 flex items-center gap-1">
                                            <FileKey className="w-3 h-3" /> Hash: {gate.attestationHash}
                                        </p>
                                    )}
                                </div>
                            </div>

                            <div className="flex items-center gap-3">
                                <span
                                    className={`text-xs px-2.5 py-1 rounded-full font-medium ${gate.passed
                                            ? 'bg-emerald-950/80 text-emerald-400 border border-emerald-800'
                                            : 'bg-rose-950/80 text-rose-400 border border-rose-800'
                                        }`}
                                >
                                    {gate.passed ? 'PASSED' : 'FAILED'}
                                </span>

                                {gate.passed && (
                                    <button
                                        onClick={() => onExportEvidence(gate.id)}
                                        className="p-2 rounded-lg bg-indigo-950/60 hover:bg-indigo-900/80 border border-indigo-800 text-indigo-300 transition-colors"
                                        title="Export DSSE Attestation"
                                    >
                                        <Download className="w-4 h-4" />
                                    </button>
                                )}
                            </div>
                        </div>
                    ))}
                </div>
            )}
        </div>
    );
};