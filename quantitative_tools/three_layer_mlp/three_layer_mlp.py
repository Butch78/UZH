import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import tensorflow as tf
from sklearn.metrics import accuracy_score
from matplotlib import colors

#Read toy data
df = pd.read_excel('toy_data.xls')
inputs = df[['x1', 'x2']]
inputs = tf.cast(inputs, tf.float32)
labels = df['y'] > 0

# Set up MLP
model_tf = tf.keras.Sequential()
model_tf.add(tf.keras.layers.Dense(3, activation='sigmoid'))
model_tf.add(tf.keras.layers.Dense(2, activation='sigmoid'))
model_tf.add(tf.keras.layers.Dense(1, activation='sigmoid'))

# Train MLP
opt = tf.optimizers.Adam(learning_rate=0.01)
model_tf.compile(optimizer = opt, loss='binary_crossentropy')
history = model_tf.fit(inputs, labels, epochs=5000, verbose=1)

# Display results
fig, axs = plt.subplots(1, 2)
axs[0].plot(history.epoch, history.history['loss'])
axs[0].set_title('error')

[t0, t1] = np.meshgrid(np.arange(0,1,0.01),np.arange(0,1,0.01))
test_data = tf.cast(np.transpose(np.array([t0, t1]).reshape(2, t0.shape[0] ** 2)), tf.float32)
t2 = np.reshape(model_tf.predict(test_data),(100,100))
axs[1].contourf(t0, t1, t2, cmap=colors.ListedColormap(['white', 'blue']), alpha = 0.4)
axs[1].scatter(df.loc[df['y']<0,'x1'], df.loc[df['y']<0,'x2'])
axs[1].set_title('prediction')
plt.show()
