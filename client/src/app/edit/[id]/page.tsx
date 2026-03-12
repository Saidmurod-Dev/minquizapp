'use client';
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { RadioGroupItem, RadioGroup } from "@/components/ui/radio-group";
import { api } from "@/lib/api";
import { ArrowLeft, Plus, Save } from "lucide-react"
import { useParams, useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export default function IdPage({ params }: { params: { id: string } }) {
    const router = useRouter()
    const param = useParams<{ id: string }>();
    const [questions, setQuestions] = useState([])
    const [testName, setTestName] = useState("")
    const [testId, setId] = useState("")
    useEffect(() => {
        setId(param?.id as string)
        console.log("Test id: ", param?.id)
        const fetchTest = async () => {
            try {
                const res = await api.get(`/test/${param?.id}`, {
                    headers: {
                        Authorization: `Bearer ${localStorage.getItem("token")}`
                    }
                });
                const data_res = res.data as any;
                console.log(data_res)
                setQuestions(JSON.parse(data_res.questions).map((q: any) => ({
                    ...q,
                    answers: q.answers || []
                })));
                setTestName(data_res?.name);
                setId(param?.id as string); // optional, just for state
            } catch (err) {
                console.error("Error fetching test:", err);
            }
        }
        fetchTest()
    }, [])

    const handleSaveTest = () => {
        const sendTest = async () => {
            const res = await api.post("/test", {
                name: testName,
                questions: JSON.stringify(questions),
                id: testId ? parseInt(testId) : null
            }, {
                headers: {
                    Authorization: `Bearer ${localStorage.getItem("token")}`
                }
            })
            if (res.data === "Unauthorized") {
                router.push("/login")
                return
            }
            console.log(res.data)

        }

        sendTest()
    }

    return <div className="w-1/2 h-20 mt-10 p-4 rounded-md mx-auto">
        <div className="flex justify-between items-center mb-6">
            <Button variant="outline" className="p-4">
                <ArrowLeft className="mr-2 h-4 w-4" />
                Back to Dashboard
            </Button>
            <Button className="p-4" onClick={handleSaveTest}>
                <Save className="mr-2 h-4 w-4" />
                Save Test
            </Button>
        </div>
        <br />
        <Card>
            <CardHeader className="space-y-3">
                <div className="flex flex-col gap-2">
                    <Label htmlFor="testname" className="text-xl">Test name</Label>
                    <Input placeholder="Enter there test name" id="testname" value={testName} onChange={(e) => setTestName(e.target.value)} />
                </div>
                <Button className="p-4 py-5"><Plus />Add question</Button>
            </CardHeader>
        </Card>
        <br />
        <div className="space-y-2">
            {questions?.map((question, index) => (
                <Card key={index}>
                    <CardHeader>
                        <div className="flex flex-col gap-2">
                            <Label htmlFor="testname" className="text-xl">Question Text</Label>
                            <Input placeholder="Enter there test name" onChange={({ target }) => setQuestions((qs: any) => qs.map((q: any, qi: any) => qi === index ? {
                                ...q,
                                text: target.value,
                            } : q))} value={question.text!} id="testname" />
                        </div>
                    </CardHeader>
                    <CardContent>
                        <div className="space-y-2">
                            <div className="flex justify-between">
                                <Label className="text-xl">Answers</Label>
                                <Button
                                    className="p-4 py-5"
                                    onClick={() => setQuestions(prev => [...prev, {
                                        text: "",
                                        answers: [{ text: "", isCorrect: true }]
                                    }])}
                                >
                                    <Plus /> Add question
                                </Button>
                            </div>
                            <div>
                                <RadioGroup value={question.answers.findIndex(a => a.isCorrect).toString()} className="w-fit" onValueChange={(val) => {
                                    const correctIndex = parseInt(val, 10);
                                    setQuestions(prev => prev.map((q, qi) => qi === index ? {
                                        ...q,
                                        answers: q.answers.map((ans, ai) => ({ ...ans, isCorrect: ai === correctIndex }))
                                    } : q))
                                }}>
                                    {question.answers.map((answer, ai) => (
                                        <div key={ai} className="flex items-center gap-2">
                                            <RadioGroupItem
                                                id={`q${index}-a${ai}`}
                                                value={ai.toString()}
                                            />
                                            <Input
                                                value={answer.text}
                                                onChange={(e) => setQuestions(prev => prev.map((q, qi) => (qi === index ? {
                                                    ...q,
                                                    answers: q.answers.map((ans, ansi) => (ansi === ai ? { ...ans, text: e.target.value } : ans))
                                                } : q)))}
                                            />
                                        </div>
                                    ))}
                                </RadioGroup>

                            </div>
                        </div>

                    </CardContent>
                </Card>
            ))}
        </div>

    </div>
}