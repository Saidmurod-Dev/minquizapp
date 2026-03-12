"use client";

import { api } from "@/lib/api";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { Card, CardHeader, CardContent, CardDescription, CardTitle, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { BarChart3, Edit, Play, PlusCircle, Trash2 } from "lucide-react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

export default function Home() {
  const [user, setUser] = useState({
    name: "",
    email: ""
  })
  const router = useRouter()
  useEffect(() => {
    const getUserData = async () => {
      try {
        const res = await api.get("/user", {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("token")}`
          }
        })
        const data_res = res.data as any;
        console.log(res)
        if (data_res == "Unauthorized") router.push("/login")
        setUser({
          name: data_res?.name,
          email: data_res?.email
        })
      } catch (err: any) {
        console.log(err)
      }
    }
    getUserData()
  }, [])
  return (
    <div className="min-h-screen m-8">
      <div className="max-w-7xl mx-auto">
        <div className="flex justify-between items-center mb-8">
          <div>
            <h1 className="text-4xl font-bold text-gray-900">Test Creator</h1>
            <p className="text-gray-600 mt-2">Welcome back, {user.name}</p>
          </div>
          <div className="flex gap-3">
            <Button size="lg" onClick={() => router.push("/edit")}>
              <PlusCircle className="mr-2 h-5 w-5" />
              Create New Test
            </Button>
          </div>
        </div>

        <Tabs defaultValue="my-tests" className="space-y-6">
          <TabsList>
            <TabsTrigger value="my-tests">My Tests (1)</TabsTrigger>
            <TabsTrigger value="public-tests">Public Tests (1)</TabsTrigger>
          </TabsList>

          <TabsContent value="my-tests" className="grid grid-cols-3 gap-2">
            <Card>
              <CardHeader>
                <CardTitle className="text-2xl">Test name</CardTitle>
              </CardHeader>
              <CardContent></CardContent>
              <CardFooter className="flex flex-col gap-2 items-start">
                <div className="flex content-between w-full">
                  <div className="flex w-full gap-2">
                    <Button variant={"outline"} className="px-4"><Play /> Take</Button>
                    <Button variant={"outline"} className="px-4"><Edit/> Edit</Button>
                  </div>
                  <Button variant={"outline"}><Trash2 /></Button>
                </div>
                <Button variant={"secondary"} className="w-full"><BarChart3 /> View Results</Button>
              </CardFooter>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle>Test name</CardTitle>
              </CardHeader>

            </Card>
          </TabsContent>

          <TabsContent value="public-tests">

          </TabsContent>
        </Tabs>

      </div>
    </div>
  );
}
