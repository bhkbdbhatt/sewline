import React from 'react';
import { PipelineExecution, Stage } from '../types';
import { CheckCircle2, XCircle, Clock, Loader2, ShieldCheck } from 'lucide-react';

interface Props {
    pipeline: PipelineExecution;
    onSelectStage: (stage: Stage) => void;
    selectedStageId?: string;
}

export const PipelineVisualizer: React.FC<Props> = ({ pipeline, onSelectStage, selectedStageId }) => {
    const getStatusIcon = (status: Stage['status']) => {
        switch (status) {
            case 'passed':
                return <CheckCircle2 className="w-5 h-5 text-emerald-400" />;
            case 'failed':
                return <XCircle className="w-5 h-5 text-rose-500" />;
            case 'running':
                return <Loader2 className="w-5 h-5 text-amber-400 animate-spin" />;
            default:
                return <Clock className="w-5 h-5 text-slate-500" />;
        }
    };

    return (
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 shadow-xl">
            <div className="flex items-center justify-between mb-6">
                <div>
                    <h2 className="text-xl font-bold text-slate-100 flex items-center gap-2">
                        {pipeline.name}
                        <span className="px-2.5 py-0.5 text-xs font-semibold bg-indigo-950 text-indigo-300 border border-indigo-800 rounded-full">
                            {pipeline.complianceProfile}
                        </span>
                    </h2>
                    <p className="text-xs text-slate-400 mt-1">ID: {pipeline.id}</p>
                </div>
            </div>

            <div className="flex items-center gap-4 overflow-x-auto pb-4">
                {pipeline.stages.map((stage, idx) => {
                    const isSelected = stage.id === selectedStageId;
                    const totalGates = stage.gates.length;
                    const passedGates = stage.gates.filter((g) => g.passed).length;

                    return (
                        <React.Fragment key={stage.id}>
                            {idx > 0 && <div className="h-0.5 w-8 bg-slate-700 shrink-0" />}

                            <button
                                onClick={() => onSelectStage(stage)}
                                className={`flex-shrink-0 w-64 p-4 rounded-lg border transition-all text-left ${isSelected
                                        ? 'bg-slate-800 border-indigo-500 ring-2 ring-indigo-500/20'
                                        : 'bg-slate-950/60 border-slate-800 hover:border-slate-700'
                                    }`}
                            >
                                <div className="flex items-center justify-between mb-2">
                                    <span className="text-xs font-mono text-slate-400 truncate max-w-[140px]">
                                        {stage.adapter}
                                    </span>
                                    {getStatusIcon(stage.status)}
                                </div>

                                <h3 className="font-semibold text-slate-200 text-sm mb-3 truncate">{stage.name}</h3>

                                <div className="flex items-center justify-between text-xs text-slate-400 pt-2 border-t border-slate-800/80">
                                    <span className="flex items-center gap-1 text-slate-300">
                                        <ShieldCheck className="w-3.5 h-3.5 text-indigo-400" />
                                        Gates: {passedGates}/{totalGates}
                                    </span>
                                    <span className="font-mono text-[10px] text-slate-500">{stage.id}</span>
                                </div>
                            </button>
                        </React.Fragment>
                    );
                })}
            </div>
        </div>
    );
};