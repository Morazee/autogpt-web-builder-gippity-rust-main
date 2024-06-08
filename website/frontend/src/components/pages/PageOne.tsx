import React, { useState } from "react";
import axios from "axios";
import "tailwindcss/tailwind.css";

function MasterPage() {
  const [registerData, setRegisterData] = useState({ username: "", password: "" });
  const [loginData, setLoginData] = useState({ username: "", password: "" });
  const [error, setError] = useState(null);

  const handleRegister = async (data) => {
    try {
      const response = await axios.post(`${process.env.REACT_APP_API_URL}/register`, data);
      // Process response from API call
    } catch (e) {
      setError(e.message);
      console.error("Registration error:", e);
    }
  };

  const handleLogin = async (data) => {
    try {
      const response = await axios.post(`${process.env.REACT_APP_API_URL}/login`, data);
      // Process response from API call
    } catch (e) {
      setError(e.message);
      console.error("Login error:", e);
    }
  };

  const fetchExternalData = async () => {
    try {
      const response = await axios.get("https://ipapi.co/json");
      // Process response from API call
    } catch (e) {
      setError(e.message);
      console.error("External API error:", e);
    }
  };

  const handleInputChange = (e, setData) => {
    const { name, value } = e.target;
    setData(prevData => ({ ...prevData, [name]: value }));
  };

  return (
    <div className="w-full">
      <section>
        <div className="px-4 py-12 bg-gradient-to-r from-purple-400 via-pink-500 to-red-500 text-center text-white font-bold">
          <h1 className="text-4xl md:text-6xl lg:text-7xl">
            Catchy title: Track your fitness progress like never before!
          </h1>
          <h2 className="mt-4 text-2xl md:text-4xl lg:text-5xl">
            Subtitle: Our state-of-the-art platform offers all the tools you need to reach your goals.
          </h2>
        </div>
        <div className="px-4 py-12 bg-gray-200 text-center text-black font-semibold">
          <p className="text-lg md:text-xl lg:text-2xl mb-6">
            Don't miss your chance to join our growing community and achieve your fitness dreams! Sign up today and take the first step towards a healthier you.
          </p>
          <button className="rounded-full py-2 px-6 text-white bg-purple-600 hover:bg-purple-700 focus:outline-none">
            Sign up now
          </button>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 px-4 py-12">
          <div className="text-center">
            <h3 className="text-2xl font-bold">Comprehensive progress tracking</h3>
            <p className="text-lg mt-4">
              Monitor your workouts, nutrition, and more to ensure you stay on track with your fitness journey.
            </p>
          </div>
          <div className="text-center">
            <h3 className="text-2xl font-bold">Personalized workout plans</h3>
            <p className="text-lg mt-4">
              Customize your training routine based on your goals, preferences, and fitness level.
            </p>
          </div>
          <div className="text-center">
            <h3 className="text-2xl font-bold">Nutritional guidance</h3>
            <p className="text-lg mt-4">
              Detailed meal plans and grocery lists to fuel your body and maximize your results.
            </p>
          </div>
        </div>
        {error && <div className="text-red-500 text-center">{error}</div>}
        <div className="px-4 py-12">
          <form
            onSubmit={(e) => {
              e.preventDefault();
              handleRegister(registerData);
            }}
          >
            <h3 className="text-2xl font-bold">Register</h3>
            <input
              type="text"
              name="username"
              placeholder="Username"
              value={registerData.username}
              onChange={(e) => handleInputChange(e, setRegisterData)}
              className="block w-full p-2 mt-4 border rounded-md"
            />
            <input
              type="password"
              name="password"
              placeholder="Password"
              value={registerData.password}
              onChange={(e) => handleInputChange(e, setRegisterData)}
              className="block w-full p-2 mt-4 border rounded-md"
            />
            <button
              type="submit"
              className="mt-4 rounded-full py-2 px-6 text-white bg-purple-600 hover:bg-purple-700 focus:outline-none"
            >
              Register
            </button>
          </form>
          <form
            onSubmit={(e) => {
              e.preventDefault();
              handleLogin(loginData);
            }}
          >
            <h3 className="text-2xl font-bold mt-8">Login</h3>
            <input
              type="text"
              name="username"
              placeholder="Username"
              value={loginData.username}
              onChange={(e) => handleInputChange(e, setLoginData)}
              className="block w-full p-2 mt-4 border rounded-md"
            />
            <input
              type="password"
              name="password"
              placeholder="Password"
              value={loginData.password}
              onChange={(e) => handleInputChange(e, setLoginData)}
              className="block w-full p-2 mt-4 border rounded-md"
            />
            <button
              type="submit"
              className="mt-4 rounded-full py-2 px-6 text-white bg-purple-600 hover:bg-purple-700 focus:outline-none"
            >
              Login
            </button>
          </form>
        </div>
      </section>
    </div>
  );
}

export default MasterPage;
