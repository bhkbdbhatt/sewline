import React, { useState } from 'react';
import { PipelineVisualizer } from './components/PipelineVisualizer';
import { GateStatusCard } from './components/GateStatusCard';
import { AgentApprovalsTable } from './components/AgentApprovalsTable';
import { PipelineExecution, Stage, ApprovalRequest } from './types';
import { Shield } from 'lucide-react';

const MOCK_PIPELINE: PipelineExecution = {
    id: "exec-2026-1005-01",
    name: "DO-178C DAL-A Flight Control System Build",
    complianceProfile: "DO-178C-DAL-A",
    status: "running",
    stages: [
        {
            id: "checkout",
            name: "Source Code Checkout",
            adapter: "adapter-github-actions",
            status: "passed",
            dependsOn: [],
            gates: []
        },
        {
            id: "requirement_trace",
            name: "Jama Requirement Traceability",
            adapter: "adapter-jama",
            status: "passed",
            dependsOn: ["checkout"],
            gates: [
                {
                    id: "gate_full_traceability",
                    name: "100% High-Level to Low-Level Requirement Traceability",
                    policyRule: "sewline.jama.full_coverage",
                    enforce: true,
                    passed: true,
                    attestationHash: "9a2f1b88e3c4d710"
                }
            ]
        },
        {
            id: "static_analysis",
            name: "Parasoft Static Analysis",
            adapter: "adapter-parasoft",
            status: "passed",
            dependsOn: ["requirement_trace"],
            gates: [
                {
                    id: "gate_zero_misra_violations",
                    name: "Zero MISRA C:2012 Violations",
                    policyRule: "sewline.parasoft.zero_violations",
                    enforce: true,
                    passed: true,
                    attestationHash: "e402a7b112f8319c"
                }
            ]
        }
    ]
};

const MOCK_APPROVALS: ApprovalRequest[] = [
    {
        approvalId: "appr_98214",
        agentId: "agent-copilot-01",
        actionDescription: "Deploy patched actuator binary to Flight Control Target Target-01",
        riskLevel: "HIGH",
        status: "PENDING",
        contextPayloadJson: "{}"
    }
];

export const App: React.FC = () => {
    const [pipeline] = useState<PipelineExecution>(MOCK_PIPELINE);
    const [selectedStage, setSelectedStage] = useState<Stage | undefined>(MOCK_PIPELINE.stages[1]);
    const [approvals, setApprovals] = useState<ApprovalRequest[]>(MOCK_APPROVALS);

    const handleExportEvidence = (gateId: string) => {
        const mockAttestation = {
            _type: "https://in-toto.io/Statement/v0.1",
            subject: [{ name: `gate-${gateId}`, digest: { sha256: "e402a7b112f8319c..." } }],
            predicateType: "https://sewline.dev/attestation/v1",
            predicate: { gateId, passed: true, timestamp: new Date().toISOString() }
        };
        const blob = new Blob([JSON.stringify(mockAttestation, null, 2)], { type: "application/json" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `dsse-attestation-${gateId}.json`;
        a.click();
    };

    const handleResolveApproval = (approvalId: string, approve: boolean) => {
        setApprovals((prev) =>
            prev.map((a) =>
                a.approvalId === approvalId ? { ...a, status: approve ? 'APPROVED' : 'REJECTED' } : a
            )
        );
    };

    return (
        <div className="min-h-screen bg-slate-950 text-slate-100 p-8 space-y-8 font-sans">
            <header className="flex items-center justify-between pb-6 border-b border-slate-800">
                <div className="flex items-center gap-3">
                    <div className="p-2.5 bg-indigo-600 rounded-xl shadow-lg shadow-indigo-600/30">
                        <Shield className="w-6 h-6 text-white" />
                    </div>
                    <div>
                        <h1 className="text-2xl font-black tracking-tight text-white">SEWLINE</h1>
                        <p className="text-xs text-slate-400">Compliance-Aware SDLC Orchestration Engine</p>
                    </div>
                </div>
            </header>

            <PipelineVisualizer
                pipeline={pipeline}
                onSelectStage={setSelectedStage}
                selectedStageId={selectedStage?.id}
            />

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <GateStatusCard stage={selectedStage} onExportEvidence={handleExportEvidence} />
                <AgentApprovalsTable approvals={approvals} onResolve={handleResolveApproval} />
            </div>
        </div>
    );
};

export default App;